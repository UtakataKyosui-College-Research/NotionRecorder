import { Command } from "commander";
import { formatToTimeZone } from "date-fns-timezone"
import {
    object,
    string,
    parse,
    union,
    literal,
    optional
} from "valibot"
import { Client } from "@notionhq/client";

const client = new Client({
    auth: process.env.NOTION_TOKEN!
})

const optionSchema = object({
    command: union([literal("start"),literal("end"),literal("check"),literal("times")]),
    title: optional(string())
})


async function NewActivity(title: string,NowDate: string) {
    const start = await client.pages.create({
        parent: {
            database_id: process.env.DATABASE_ID!,
            type: "database_id"
        },
        properties: {
            "名前": {
                type: "title",
                title: [
                    {
                        text: {
                            content: title
                        }
                    }
                ]
            },
            "開始時刻": {
                type: "date",
                date: {
                    start: NowDate,
                    time_zone:  "Asia/Tokyo"
                }
            }
        }
    })
    if(start.id){
        console.log(`Success! new Activity ${NowDate}`)
    }else {
        console.error("Add Error")
    }
}

async function EndActivity(NowDate: string) {
    const history = await client.databases.query({
        database_id: process.env.DATABASE_ID!,
        filter: {
            and: [
                {
                    property: "開始時刻",
                    type: "date",
                    date: {
                        equals: formatToTimeZone(new Date(),"YYYY-MM-DD",{timeZone: "Asia/Tokyo"})
                    }
                    
                }
            ]
        }
    })
    if(history.results.length !== 0){
        const item  = history.results.shift()
        if(item === undefined){
            throw new Error("今日の活動は開始されていません")
        }else {
            const end = await client.pages.update({
                page_id: item.id,
                properties: {
                    "終了時刻": {
                        type: "date",
                        date: {
                            start: NowDate,
                            time_zone: "Asia/Tokyo"
                        }
                    }
                }
            })
            if(end.id){
                console.log(`Success! new Activity ${NowDate}`)
            }else {
                console.error("Add Error")
            }
            console.log(`Congrats! Activity Ended ${NowDate}`)
        }
    }else {
        console.error("今日の活動は開始されていません")
    }
}
  
const commandline = new Command("Notion-Recorder")

commandline
    .description("研究のための記録を残すためのCLIです")
    .version("v1.0.0")
    .option("-c, --command <command>","使用するコマンドを指定する")
    .option("-t, --title <title>","記録開始、記録終了するときのデータのタイトルを指定")
    .action(async(option) => {
        const parsedOption = parse(optionSchema,option)
        // console.log(`${parsedOption.command}`,`${parsedOption.title}`)
        // const Format = "YYYY-MM-DD HH:mm:ss
        const NowDate = formatToTimeZone(new Date(),"YYYY-MM-DD HH:mm:ss",{timeZone: "Asia/Tokyo"})
        // console.log(NowDate)
        const {command,title} = parsedOption
        if(command !== "times" && command !== "check") {
            if(title === undefined){
                throw new Error("引数が足りません")
            }else {
                console.log(NowDate + ": " + title)
                if(command === "start"){
                    NewActivity(title,NowDate)
                }
                else if(command === "end") {
                    EndActivity(NowDate)
                }
            }
        }else {
            if(command === "check"){
                const item = await client.databases.query({
                    database_id: process.env.DATABASE_ID!,
                    filter: {
                        and: [
                            {
                                property: "開始時刻",
                                type: "date",
                                date: {
                                    equals: formatToTimeZone(new Date(),"YYYY-MM-DD",{timeZone: "Asia/Tokyo"})
                                }
                            }
                        ]
                    }
                })
                console.log(item.results.length === 0 ? "まだ今日は活動できていません" : "今日は活動の記録があります")
            }else {
                console.log(command,"は、まだ実装されてません！")
            }
           
        }
    })
commandline.parse()

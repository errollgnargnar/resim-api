import { useEffect, useState } from "react"

export default function Blockie ({data}) {

    const [blockieStyle, setBlockieStyle] = useState({});

    useEffect(() => {
        if (data.includes('Packages') || data.includes('Components') || data.includes('Resource Managers')) {
            setBlockieStyle(styles.divider)
        } else {
            setBlockieStyle(styles.data);
        }
    }, [data])

    const styles = {
        divider: {
            borderRadius:"15px",
            width:"50%",
            border:"1px solid gray",
            backgroundColor:"beige",
            overflowWrap:"break-word"
        },
        data: {
            borderRadius:"15px",
            border:"1px solid gray",
            overflowWrap:"break-word"
        }
    }

    return (
        <div style={blockieStyle}>
            {data}
        </div>
    )
}
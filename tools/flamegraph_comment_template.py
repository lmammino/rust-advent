import os.path
import sys

github_repository = sys.argv[1]
hash = sys.argv[2]

for filename in sys.argv[3:]:
    summary = os.path.basename(filename)
    summary = summary[: summary.rfind(".")]
    print(
        f"""
<details>
    <summary>{ summary }</summary>                        
    <p>
    <a target="_blank" rel="noopener noreferrer" 
                href="https://raw.githubusercontent.com/{ github_repository }/{ hash }/{ filename }">
            <img src="https://raw.githubusercontent.com/{ github_repository }/{ hash }/{ filename }" 
                alt="" style="max-width:100%;" />
    </a>
    </p>
</details>"""
    )

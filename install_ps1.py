import os
import subprocess
from pathlib import Path

def configure_powershell_profile():
    try:
        get_profile = subprocess.run(
            ["powershell.exe", "-Command", "echo $PROFILE.CurrentUserAllHosts"],
            check=True,
            capture_output=True,
            text=True
        )
        profile_path = Path(get_profile.stdout.strip())
        
        source_script = r'./powershell/profile.ps1'
        
        profile_path.parent.mkdir(parents=True, exist_ok=True)
        
        if not profile_path.exists():
            profile_path.touch()
            
        with open(source_script, 'r', encoding='utf-8') as src, \
             open(profile_path, 'a', encoding='utf-8') as dst:
            dst.write(src.read())
            
        print(f"成功更新配置文件：{profile_path}")
        
    except subprocess.CalledProcessError as e:
        print(f"获取PowerShell路径失败: {e.stderr}")
    except FileNotFoundError:
        print("找不到源脚本文件")
    except Exception as e:
        print(f"操作失败: {str(e)}")

if __name__ == "__main__":
    configure_powershell_profile()

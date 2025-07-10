#!/usr/bin/env python3
"""
🏗️ Video Server 代码导览工具
Interactive Code Tour for Video Server Architecture
"""

import os
import subprocess
import sys
from pathlib import Path

class CodeTour:
    def __init__(self):
        self.project_root = Path.cwd()
        self.files_to_show = {
            "🚀 应用启动": "src/main.rs",
            "⚙️ 配置系统": "src/config.rs", 
            "📡 API处理": "src/api/handlers.rs",
            "🎥 视频服务": "src/services/video/snapshot.rs",
            "📊 数据模型": "src/models/app_state.rs",
            "🔧 中间件": "src/api/middleware.rs",
            "🛠️ 工具函数": "src/utils/system.rs",
        }
        
    def show_banner(self):
        print("\n" + "="*60)
        print("🏗️  VIDEO SERVER 代码导览")
        print("="*60)
        print("📖 快速理解项目架构和核心代码")
        print("🎯 按数字选择要查看的模块")
        print("💡 建议按顺序阅读以便更好理解")
        print("="*60)
    
    def show_menu(self):
        print("\n📋 可用模块:")
        for i, (name, file_path) in enumerate(self.files_to_show.items(), 1):
            lines = self.count_lines(file_path)
            print(f"  {i}. {name:<15} ({file_path}) - {lines} 行")
        
        print(f"\n  0. 🌲 显示项目结构")
        print(f"  a. 📚 显示架构文档")
        print(f"  t. 🧪 运行快速测试")
        print(f"  q. 退出")
    
    def count_lines(self, file_path):
        try:
            with open(self.project_root / file_path, 'r', encoding='utf-8') as f:
                return len(f.readlines())
        except:
            return "?"
    
    def show_file_content(self, file_path, description):
        full_path = self.project_root / file_path
        if not full_path.exists():
            print(f"❌ 文件不存在: {file_path}")
            return
            
        print(f"\n{'='*60}")
        print(f"📄 {description}")
        print(f"📁 文件: {file_path}")
        print(f"{'='*60}")
        
        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # 显示文件内容，带行号
            lines = content.split('\n')
            total_lines = len(lines)
            
            print(f"📊 总行数: {total_lines}")
            print("-" * 60)
            
            for i, line in enumerate(lines[:50], 1):  # 只显示前50行
                print(f"{i:3d} | {line}")
            
            if total_lines > 50:
                print(f"... (还有 {total_lines - 50} 行)")
                print(f"💡 完整内容请查看: {file_path}")
                
        except Exception as e:
            print(f"❌ 读取文件出错: {e}")
    
    def show_project_structure(self):
        print(f"\n{'='*60}")
        print("🌲 项目结构")
        print(f"{'='*60}")
        
        try:
            # 使用tree命令显示项目结构
            result = subprocess.run(['tree', '-I', 'target|node_modules|.git'], 
                                  capture_output=True, text=True, cwd=self.project_root)
            if result.returncode == 0:
                print(result.stdout)
            else:
                # 如果tree命令不可用，使用简单的目录列表
                self._show_directory_structure()
        except FileNotFoundError:
            # tree命令不存在，使用Python实现
            self._show_directory_structure()
    
    def _show_directory_structure(self):
        """简单的目录结构显示"""
        def show_dir(path, prefix=""):
            items = sorted(path.iterdir())
            dirs = [item for item in items if item.is_dir() and not item.name.startswith('.')]
            files = [item for item in items if item.is_file() and not item.name.startswith('.')]
            
            for i, dir_path in enumerate(dirs):
                is_last_dir = (i == len(dirs) - 1) and len(files) == 0
                print(f"{prefix}{'└── ' if is_last_dir else '├── '}{dir_path.name}/")
                extension = "    " if is_last_dir else "│   "
                if dir_path.name not in ['target', 'node_modules', '.git']:
                    show_dir(dir_path, prefix + extension)
            
            for i, file_path in enumerate(files[:10]):  # 限制文件显示数量
                is_last = i == len(files) - 1
                print(f"{prefix}{'└── ' if is_last else '├── '}{file_path.name}")
        
        print(f"{self.project_root.name}/")
        show_dir(self.project_root)
    
    def show_architecture_doc(self):
        arch_file = self.project_root / "ARCHITECTURE.md"
        if arch_file.exists():
            self.show_file_content("ARCHITECTURE.md", "📚 架构文档")
        else:
            print("❌ 架构文档不存在")
    
    def run_quick_tests(self):
        print(f"\n{'='*60}")
        print("🧪 运行快速测试")
        print(f"{'='*60}")
        
        tests = [
            ("📋 检查代码编译", "cargo check"),
            ("🔧 代码格式检查", "cargo fmt --check"),
            ("🚀 启动服务器 (后台)", "cargo run &"),
        ]
        
        for description, command in tests:
            print(f"\n{description}:")
            print(f"💻 命令: {command}")
            
            if "cargo run" in command:
                print("⚠️  服务器将在后台启动，使用 Ctrl+C 或 pkill 停止")
                user_input = input("是否继续? (y/N): ")
                if user_input.lower() != 'y':
                    continue
            
            try:
                if "&" in command:
                    # 后台运行
                    subprocess.Popen(command.replace(" &", "").split(), 
                                   cwd=self.project_root)
                    print("✅ 后台启动成功")
                else:
                    result = subprocess.run(command.split(), 
                                          capture_output=True, text=True, 
                                          cwd=self.project_root, timeout=30)
                    if result.returncode == 0:
                        print("✅ 成功")
                        if result.stdout:
                            print(result.stdout[:500])  # 限制输出长度
                    else:
                        print("❌ 失败")
                        print(result.stderr[:500])
                        
            except subprocess.TimeoutExpired:
                print("⏱️ 超时")
            except Exception as e:
                print(f"❌ 错误: {e}")
    
    def run(self):
        self.show_banner()
        
        while True:
            self.show_menu()
            choice = input("\n🎯 请选择 (1-7, 0, a, t, q): ").strip().lower()
            
            if choice == 'q':
                print("👋 再见!")
                break
            elif choice == '0':
                self.show_project_structure()
            elif choice == 'a':
                self.show_architecture_doc()
            elif choice == 't':
                self.run_quick_tests()
            elif choice.isdigit() and 1 <= int(choice) <= len(self.files_to_show):
                idx = int(choice) - 1
                name, file_path = list(self.files_to_show.items())[idx]
                self.show_file_content(file_path, name)
            else:
                print("❌ 无效选择，请重试")
            
            input("\n按 Enter 继续...")

if __name__ == "__main__":
    if not Path("Cargo.toml").exists():
        print("❌ 请在 video-server 项目根目录运行此脚本")
        sys.exit(1)
    
    tour = CodeTour()
    tour.run() 
# Introduction 
TODO: Give a short introduction of your project. Let this section explain the objectives or the motivation behind this project. 

# Getting Started
TODO: Guide users through getting your code up and running on their own system. In this section you can talk about:
1.	Installation process
2.	Software dependencies
3.	Latest releases
4.	API references

# Build and Test
TODO: Describe and show how to build your code and run the tests. 

# Contribute
TODO: Explain how other users and developers can contribute to make your code better. 

If you want to learn more about creating good readme files then refer the following [guidelines](https://docs.microsoft.com/en-us/azure/devops/repos/git/create-a-readme?view=azure-devops). You can also seek inspiration from the below readme files:
- [ASP.NET Core](https://github.com/aspnet/Home)
- [Visual Studio Code](https://github.com/Microsoft/vscode)
- [Chakra Core](https://github.com/Microsoft/ChakraCore)

# Git command-line

To get your local changes
```
!git log  --decorate --parents -p -m -t HEAD --not origin/master^ > log.mod.log
```

To get status of your repo and get tree view of your local and origin branches
```
!git status -vv > log.diff.log ; git log --branches --decorate --parents --oneline --graph --not origin/master^ > log.log
```

To see your branches
```
!git branch -a | sed -n -r -e '/lidener/ {s/^[^[:alnum:]]+//;s/remotes\/origin\///;p}' | sort -u > log.diff.log
```
## Contributing to Ngyn

Contributing to open source projects like Ngyn is a rewarding way to learn, teach, and build experience. Not only do you get to expand your knowledge and skills, you also get to be part of a community that is working together to build something great. Whether you're a seasoned open source contributor or looking to make your first contribution, we appreciate your efforts and look forward to learning from you.

## How to Contribute

Contributing to Ngyn involves a few common steps. This guide will walk you through the process, from forking the repository to submitting your pull request. Let's get started!

1. **Fork the repository**: Navigate to the [Ngyn GitHub repository](https://github.com/ngyn-rs/ngyn-rs), and click on the 'Fork' button in the upper right corner. This will create a copy of the repository in your GitHub account.

2. **Clone the repository**: Open a terminal on your local machine, navigate to the directory where you want to clone the repository, and run the following command:

   ```
   git clone https://github.com/<your-username>/ngyn-rs.git
   ```

   Replace `<your-username>` with your GitHub username.

3. **Create a new branch**: Navigate into the cloned repository and create a new branch for your changes:

   ```
   cd ngyn
   git checkout -b my-feature
   ```

   Replace `my-feature` with a descriptive name for your branch.

4. **Make your changes**: Edit the code, add new features, fix bugs, or make other improvements. Make sure to follow the Ngyn coding standards and guidelines.

5. **Commit your changes**: Once you're satisfied with your changes, stage and commit them using the [Conventional Commits](https://www.conventionalcommits.org/) standard:

   ```
   git add .
   git commit -m "feat: Add my feature"
   ```

   Replace `"feat: Add my feature"` with a descriptive commit message following the Conventional Commits standard. For example, if you've fixed a bug, you might use `"fix: Corrected bug in feature X"`.

6. **Push your changes**: Push your changes to your forked repository:

   ```
   git push origin my-feature
   ```

7. **Open a pull request**: Navigate back to your forked repository on GitHub, select your new branch, and click the 'New pull request' button. Fill out the pull request form, and then submit it.

8. **Review and Address Comments**: Once your pull request is submitted, it will be reviewed by the Ngyn maintainers. Address any comments or requests for changes from the maintainers in a timely manner.

9. **Merge your changes**: If your changes are approved, they will be merged into the main codebase. Congratulations, you've just contributed to Ngyn! Remember to delete your branch after it has been merged.

10. **Sync your Fork**: After your changes have been merged, remember to sync your fork with the original repository to keep it up-to-date.

```
git checkout dev
git pull upstream dev
git push origin dev
```

This will ensure your fork is always updated with the latest changes from the `dev` branch.

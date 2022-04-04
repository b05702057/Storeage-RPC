

# Steps to set up EC2 instance for Labs

## UCSD AWS Student account with AWS credit:

Go to: http://awsed.ucsd.edu/, and select class `CSE223B_SP22_A00` to sign in to
get to AWS console. Since this is a restricted account, you will have very
limited permissions so unless you follow proper steps, you are going to hit
errors that usually do not make sense.

1. Make sure you selected the Oregon (`us-west-2`) region. That seems to be the
   safe one for now.
2. Click [Launch Instance] and follow the quick start guide to start a vm
    1. Select Ubuntu Server 20.04 LTS
    2. Choose a deployment. t3.micro is cheap and sufficient for lab1
        - note, if you choose to use rust-analyzer with vscode as your
            development environment, you might need a larger instance type to
            accomodate rust-analyzer's footprint
    3. Edit the security group to be `CSE223B_SP21`
3. Once launched, go to the EC2 dashboard. There will a big list of instances
   available for your selection, you can find yours by the "Key Name" or "Launch
   Time" columns.
4. SSH into your VM using it's public IPv4 DNS address.
5. In the VM, clone the Lab1 repo to your VM, you will likely need to install
   git: `apt install git`.
6. Run the `aws-ubuntu-install.sh` script from the root directory of your lab 1
   repository.
7. Test your installation by running `cargo check`.

## Notes

1. [Launch EC2 instance from custom
   image](https://aws.amazon.com/premiumsupport/knowledge-center/launch-instance-custom-ami)
2. [Instructions on how to create an EC2
   image](https://docs.aws.amazon.com/quickstarts/latest/vmlaunch/step-1-launch-instance.html)


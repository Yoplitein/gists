#!/usr/bin/env python2
"""
Updater script for freedns.afraid.org
(Should work with other services that have you simply GET a URL to update)

Also sends you emails if something's wrong or when your IP is updated,
provided you have a local email server.
Additionally, passing --cron as an argument will let cron handle the email.

Suggested crontab entry:
0 0,12 * * * /usr/local/bin/update-dyndns
"""
import urllib2, time, syslog, smtplib, sys
from email.mime.text import MIMEText

#settings
URL = "https://freedns.afraid.org/dynamic/update.php?key"

##email settings
SEND_EMAIL = True
EMAIL_FROM = "crond <cron@>"
EMAIL_TO   = ""

EMAIL_CRON = (True in [(x in sys.argv) for x in ["-c", "--cron"]])

def doRequest():
    req = urllib2.urlopen(URL)
    return req.getcode(), req.read()

def sendEmail(subject, body):
    message = MIMEText(body)
    message["Subject"] = subject
    message["From"] = EMAIL_FROM
    message["To"] = EMAIL_TO

    s = smtplib.SMTP("127.0.0.1")
    s.sendmail(EMAIL_FROM, [EMAIL_TO], message.as_string())
    s.quit()

def log(message, error=False, emailOverride=False):
    if emailOverride or not SEND_EMAIL:
        syslog.syslog("[update-dyndns] %s" % message)
    elif EMAIL_CRON:
        print "[update-dyndns]", message
    else:
        subject = "update-dyndns "

        if error:
            subject += "Error"
        else:
            subject += "Info"

        sendEmail(subject, message)

def main():
    try:
        code, response = doRequest()
    except (urllib2.HTTPError, urllib2.URLError) as e:
        reason = str(e)

        log("Error: %s" % reason, error=True)
        sys.exit(1)

    _resp = response.lower()

    #error check
    if True in [x in _resp for x in ["bad request", "unable to locate", "invalid update"]]:
        log("Got following while updating DynDNS: %s" % response, error=True)
        sys.exit(1)

    #update check
    if "updated" in _resp:
        log("DynDNS address updated. Message: %s" % response)
        sys.exit()

    log("[update-dyndns] Address has not changed", emailOverride=True)

if __name__ == "__main__":
    main()
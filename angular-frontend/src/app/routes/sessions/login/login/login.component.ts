import axios from 'axios'

import { Component, inject } from '@angular/core'
import * as md5 from 'md5'
import { CookieService } from 'ngx-cookie'
import { Router } from '@angular/router'

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.sass'],
  // providers: [CookieService]
})
export class LoginComponent {
  // cookieService = inject(CookieService)
  constructor(
    private readonly cookieService: CookieService,
    private readonly router: Router, 
  ) {}

  private readonly loginCheck: Function = () => { return this.loginMsg.name != '' && this.loginMsg.password != '' }
  private readonly changeJudge: Function = () => { this.allowLogin = this.loginCheck() } 

  allowLogin: boolean = false

  loginMsg: { name: string, password: string } = { name: '', password: '' }
  shownMsg: { btnMsg: string } = { btnMsg: '登录' }
  styles: { btnStyle: string } = { btnStyle: 'primary' }

  changeName: Function = (e: string) => {
    this.loginMsg.name = e
    this.changeJudge()
  }
  changePassword: Function = (e: string) => {
    this.loginMsg.password = md5(e)
    this.changeJudge()
  }

  confirmLogin: Function = async () => {
    if (! this.loginCheck()) return
    try {
      var result = await axios({
        url: '/api/user/',
        method: 'post',
        data: this.loginMsg
      })
      // ...write in cookie
      if (result.data.length != 0) {
        var date = new Date()
        date.setSeconds(date.getSeconds() + 60 * 60 * 24 * 3)
        console.log(date.toDateString()) 
        this.cookieService.put("user", JSON.stringify(result.data[0]), {
          path: '/',
          expires: date,
        })
        this.cookieService.get("user")
        // this.styles.btnClass = 'btn-success'
        this.shownMsg.btnMsg = '登录成功'
        setTimeout(() => {this.router.navigateByUrl("/")}, 1500)
      } else {
        this.styles.btnStyle = 'warn'
        this.shownMsg.btnMsg = '用户不存在或密码错误'
      }
    } catch (err) {
      this.styles.btnStyle = 'warn'
      this.shownMsg.btnMsg = '发生错误，请联系管理员'
      console.error(err)
    }
  }
}

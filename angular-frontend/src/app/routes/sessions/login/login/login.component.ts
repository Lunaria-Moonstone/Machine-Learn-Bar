import axios from 'axios'
import { Component } from '@angular/core'

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.sass']
})
export class LoginComponent {
  
  private readonly loginCheck: Function = () => this.loginMsg.name != '' && this.loginMsg.password != ''

  allowLogin: boolean = false

  loginMsg: { name: string, password: string } = { name: '', password: '' }

  confirmLogin: Function = async () => {
    if (! this.loginCheck()) return

    try {
      var user = await axios({
        url: '/api/user/',
        method: 'post',
        data: this.loginMsg
      })
      // ...write in cookie
    } catch (err) {
      console.error(err)
    }
  }
}

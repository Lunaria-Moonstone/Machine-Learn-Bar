import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  // templateUrl: './app.component.html',
  template: '<router-outlet></router-outlet>',
  // styles: [`
  //   * 
  //     padding: 0
  //     margin: 0
  // `]
  styleUrls: ['./app.component.sass']
})
export class AppComponent {
  title = 'angular-frontend';
}

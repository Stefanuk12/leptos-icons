use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle r = "3" cx = "16.6" cy = "15.89" ></ circle > < circle cy = "7.4" cx = "8.11" r = "3" ></ circle > < circle cy = "11.65" cx = "12.35" r = "3" ></ circle > < circle cy = "5.85" cx = "13.91" r = "3" ></ circle > < circle cy = "10.09" r = "3" cx = "18.15" ></ circle > < circle r = "3" cx = "6.56" cy = "13.2" ></ circle > < circle cy = "17.44" r = "3" cx = "10.8" ></ circle > < circle r = "3" cy = "19" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;
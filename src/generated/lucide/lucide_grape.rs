use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cx = "16.6" cy = "15.89" r = "3" ></ circle > < circle cx = "8.11" r = "3" cy = "7.4" ></ circle > < circle cx = "12.35" cy = "11.65" r = "3" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle cy = "10.09" cx = "18.15" r = "3" ></ circle > < circle cy = "13.2" cx = "6.56" r = "3" ></ circle > < circle cx = "10.8" cy = "17.44" r = "3" ></ circle > < circle cx = "5" r = "3" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;
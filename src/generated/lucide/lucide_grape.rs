use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle r = "3" cx = "16.6" cy = "15.89" ></ circle > < circle r = "3" cx = "8.11" cy = "7.4" ></ circle > < circle cy = "11.65" cx = "12.35" r = "3" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle cx = "18.15" cy = "10.09" r = "3" ></ circle > < circle cy = "13.2" r = "3" cx = "6.56" ></ circle > < circle cy = "17.44" cx = "10.8" r = "3" ></ circle > < circle cy = "19" cx = "5" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;
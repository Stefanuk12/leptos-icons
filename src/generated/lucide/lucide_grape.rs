use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle r = "3" cy = "15.89" cx = "16.6" ></ circle > < circle cy = "7.4" cx = "8.11" r = "3" ></ circle > < circle cy = "11.65" r = "3" cx = "12.35" ></ circle > < circle cy = "5.85" cx = "13.91" r = "3" ></ circle > < circle r = "3" cy = "10.09" cx = "18.15" ></ circle > < circle r = "3" cx = "6.56" cy = "13.2" ></ circle > < circle cx = "10.8" cy = "17.44" r = "3" ></ circle > < circle cx = "5" r = "3" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor")] } ;
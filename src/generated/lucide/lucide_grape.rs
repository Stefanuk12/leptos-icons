use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cy = "15.89" r = "3" cx = "16.6" ></ circle > < circle cy = "7.4" cx = "8.11" r = "3" ></ circle > < circle cy = "11.65" cx = "12.35" r = "3" ></ circle > < circle cx = "13.91" r = "3" cy = "5.85" ></ circle > < circle cx = "18.15" r = "3" cy = "10.09" ></ circle > < circle cx = "6.56" r = "3" cy = "13.2" ></ circle > < circle cx = "10.8" r = "3" cy = "17.44" ></ circle > < circle cx = "5" r = "3" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cx = "16.6" r = "3" cy = "15.89" ></ circle > < circle cy = "7.4" cx = "8.11" r = "3" ></ circle > < circle cx = "12.35" r = "3" cy = "11.65" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle cy = "10.09" cx = "18.15" r = "3" ></ circle > < circle cy = "13.2" cx = "6.56" r = "3" ></ circle > < circle cx = "10.8" cy = "17.44" r = "3" ></ circle > < circle cx = "5" cy = "19" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;
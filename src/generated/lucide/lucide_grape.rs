use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cy = "15.89" cx = "16.6" r = "3" ></ circle > < circle cx = "8.11" cy = "7.4" r = "3" ></ circle > < circle r = "3" cy = "11.65" cx = "12.35" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle cy = "10.09" r = "3" cx = "18.15" ></ circle > < circle cy = "13.2" cx = "6.56" r = "3" ></ circle > < circle r = "3" cx = "10.8" cy = "17.44" ></ circle > < circle r = "3" cx = "5" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;
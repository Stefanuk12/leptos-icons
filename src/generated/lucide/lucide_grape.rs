use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle r = "3" cx = "16.6" cy = "15.89" ></ circle > < circle cy = "7.4" r = "3" cx = "8.11" ></ circle > < circle cx = "12.35" r = "3" cy = "11.65" ></ circle > < circle r = "3" cx = "13.91" cy = "5.85" ></ circle > < circle r = "3" cx = "18.15" cy = "10.09" ></ circle > < circle cx = "6.56" cy = "13.2" r = "3" ></ circle > < circle cx = "10.8" cy = "17.44" r = "3" ></ circle > < circle r = "3" cy = "19" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle r = "3" cx = "16.6" cy = "15.89" ></ circle > < circle cx = "8.11" r = "3" cy = "7.4" ></ circle > < circle cy = "11.65" r = "3" cx = "12.35" ></ circle > < circle cy = "5.85" r = "3" cx = "13.91" ></ circle > < circle cy = "10.09" cx = "18.15" r = "3" ></ circle > < circle cx = "6.56" cy = "13.2" r = "3" ></ circle > < circle r = "3" cy = "17.44" cx = "10.8" ></ circle > < circle cx = "5" cy = "19" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
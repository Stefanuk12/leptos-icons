use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cy = "15.89" cx = "16.6" r = "3" ></ circle > < circle cy = "7.4" r = "3" cx = "8.11" ></ circle > < circle cx = "12.35" r = "3" cy = "11.65" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle r = "3" cy = "10.09" cx = "18.15" ></ circle > < circle cy = "13.2" cx = "6.56" r = "3" ></ circle > < circle cx = "10.8" r = "3" cy = "17.44" ></ circle > < circle cy = "19" r = "3" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle r = "3" cx = "16.6" cy = "15.89" ></ circle > < circle cx = "8.11" cy = "7.4" r = "3" ></ circle > < circle cy = "11.65" r = "3" cx = "12.35" ></ circle > < circle r = "3" cy = "5.85" cx = "13.91" ></ circle > < circle cx = "18.15" cy = "10.09" r = "3" ></ circle > < circle cy = "13.2" r = "3" cx = "6.56" ></ circle > < circle cy = "17.44" r = "3" cx = "10.8" ></ circle > < circle cx = "5" cy = "19" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;
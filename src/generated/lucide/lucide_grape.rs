use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cx = "16.6" cy = "15.89" r = "3" ></ circle > < circle cx = "8.11" cy = "7.4" r = "3" ></ circle > < circle cx = "12.35" r = "3" cy = "11.65" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle r = "3" cy = "10.09" cx = "18.15" ></ circle > < circle cx = "6.56" cy = "13.2" r = "3" ></ circle > < circle r = "3" cx = "10.8" cy = "17.44" ></ circle > < circle r = "3" cy = "19" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;
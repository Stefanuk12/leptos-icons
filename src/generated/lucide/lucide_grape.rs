use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cy = "15.89" cx = "16.6" r = "3" ></ circle > < circle cx = "8.11" cy = "7.4" r = "3" ></ circle > < circle cx = "12.35" cy = "11.65" r = "3" ></ circle > < circle cx = "13.91" cy = "5.85" r = "3" ></ circle > < circle cx = "18.15" cy = "10.09" r = "3" ></ circle > < circle r = "3" cy = "13.2" cx = "6.56" ></ circle > < circle r = "3" cx = "10.8" cy = "17.44" ></ circle > < circle cx = "5" cy = "19" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;
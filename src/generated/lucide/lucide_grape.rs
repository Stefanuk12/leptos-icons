use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cy = "15.89" r = "3" cx = "16.6" ></ circle > < circle r = "3" cx = "8.11" cy = "7.4" ></ circle > < circle cx = "12.35" cy = "11.65" r = "3" ></ circle > < circle cx = "13.91" r = "3" cy = "5.85" ></ circle > < circle cx = "18.15" r = "3" cy = "10.09" ></ circle > < circle cy = "13.2" cx = "6.56" r = "3" ></ circle > < circle cy = "17.44" r = "3" cx = "10.8" ></ circle > < circle cx = "5" cy = "19" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cx = "16.6" r = "3" cy = "15.89" ></ circle > < circle cx = "8.11" r = "3" cy = "7.4" ></ circle > < circle cy = "11.65" r = "3" cx = "12.35" ></ circle > < circle cx = "13.91" r = "3" cy = "5.85" ></ circle > < circle cx = "18.15" r = "3" cy = "10.09" ></ circle > < circle cy = "13.2" r = "3" cx = "6.56" ></ circle > < circle r = "3" cx = "10.8" cy = "17.44" ></ circle > < circle cx = "5" cy = "19" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;
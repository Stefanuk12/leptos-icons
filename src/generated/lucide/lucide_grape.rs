use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cx = "16.6" cy = "15.89" r = "3" ></ circle > < circle cx = "8.11" cy = "7.4" r = "3" ></ circle > < circle r = "3" cx = "12.35" cy = "11.65" ></ circle > < circle cx = "13.91" r = "3" cy = "5.85" ></ circle > < circle cy = "10.09" cx = "18.15" r = "3" ></ circle > < circle r = "3" cy = "13.2" cx = "6.56" ></ circle > < circle cx = "10.8" r = "3" cy = "17.44" ></ circle > < circle cy = "19" cx = "5" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cy = "15.89" cx = "16.6" r = "3" ></ circle > < circle cy = "7.4" cx = "8.11" r = "3" ></ circle > < circle cx = "12.35" cy = "11.65" r = "3" ></ circle > < circle cy = "5.85" r = "3" cx = "13.91" ></ circle > < circle r = "3" cx = "18.15" cy = "10.09" ></ circle > < circle cx = "6.56" cy = "13.2" r = "3" ></ circle > < circle r = "3" cx = "10.8" cy = "17.44" ></ circle > < circle cy = "19" cx = "5" r = "3" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;
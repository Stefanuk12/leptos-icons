use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 5V2l-5.89 5.89" ></ path > < circle cx = "16.6" r = "3" cy = "15.89" ></ circle > < circle cy = "7.4" cx = "8.11" r = "3" ></ circle > < circle cy = "11.65" cx = "12.35" r = "3" ></ circle > < circle r = "3" cy = "5.85" cx = "13.91" ></ circle > < circle cx = "18.15" cy = "10.09" r = "3" ></ circle > < circle r = "3" cx = "6.56" cy = "13.2" ></ circle > < circle cx = "10.8" cy = "17.44" r = "3" ></ circle > < circle cy = "19" r = "3" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRAPE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
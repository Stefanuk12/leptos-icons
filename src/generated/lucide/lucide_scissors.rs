use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "6" cy = "6" ></ circle > < path d = "M8.12 8.12 12 12" ></ path > < path d = "M20 4 8.12 15.88" ></ path > < circle cy = "18" r = "3" cx = "6" ></ circle > < path d = "M14.8 14.8 20 20" ></ path > < / > } } pub const LUCIDE_SCISSORS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" cx = "6" r = "3" ></ circle > < path d = "M8.12 8.12 12 12" ></ path > < path d = "M20 4 8.12 15.88" ></ path > < circle cy = "18" cx = "6" r = "3" ></ circle > < path d = "M14.8 14.8 20 20" ></ path > < / > } } pub const LUCIDE_SCISSORS : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;
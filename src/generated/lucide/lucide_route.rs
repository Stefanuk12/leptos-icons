use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "19" cx = "6" r = "3" ></ circle > < path d = "M9 19h8.5a3.5 3.5 0 0 0 0-7h-11a3.5 3.5 0 0 1 0-7H15" ></ path > < circle cx = "18" r = "3" cy = "5" ></ circle > < / > } } pub const LUCIDE_ROUTE : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;
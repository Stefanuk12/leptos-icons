use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cx = "12" cy = "7" ></ circle > < / > } } pub const LUCIDE_USER : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M8 15h8" ></ path > < path d = "M8 9h2" ></ path > < path d = "M14 9h2" ></ path > < / > } } pub const LUCIDE_ANNOYED : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle r = "3" cx = "12" cy = "10" ></ circle > < path d = "M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662" ></ path > < / > } } pub const LUCIDE_CIRCLE_USER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;
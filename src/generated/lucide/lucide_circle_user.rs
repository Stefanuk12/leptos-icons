use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < circle r = "3" cx = "12" cy = "10" ></ circle > < path d = "M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662" ></ path > < / > } } pub const LUCIDE_CIRCLE_USER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.814.014L6 21" ></ path > < path d = "m14 19.5 3-3 3 3" ></ path > < path d = "M17 22v-5.5" ></ path > < circle cx = "9" cy = "9" r = "2" ></ circle > < / > } } pub const LUCIDE_IMAGE_UP : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;
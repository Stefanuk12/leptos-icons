use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" ></ path > < circle r = "4" cy = "7" cx = "12" ></ circle > < / > } } pub const LUCIDE_USER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;
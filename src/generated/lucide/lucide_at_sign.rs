use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "4" ></ circle > < path d = "M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8" ></ path > < / > } } pub const LUCIDE_AT_SIGN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;
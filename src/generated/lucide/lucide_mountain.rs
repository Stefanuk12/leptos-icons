use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8 3 4 8 5-5 5 15H2L8 3z" ></ path > < / > } } pub const LUCIDE_MOUNTAIN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;
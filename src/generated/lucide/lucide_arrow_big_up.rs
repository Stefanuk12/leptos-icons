use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18v-6H5l7-7 7 7h-4v6H9z" ></ path > < / > } } pub const LUCIDE_ARROW_BIG_UP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;
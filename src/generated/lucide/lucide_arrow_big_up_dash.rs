use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 19h6" ></ path > < path d = "M9 15v-3H5l7-7 7 7h-4v3H9z" ></ path > < / > } } pub const LUCIDE_ARROW_BIG_UP_DASH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z" ></ path > < / > } } pub const LUCIDE_FACEBOOK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;
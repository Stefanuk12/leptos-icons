use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4a6.5 6.5 0 1 0-4-4Z" ></ path > < circle cy = "7.5" fill = "currentColor" cx = "16.5" r = ".5" ></ circle > < / > } } pub const LUCIDE_KEY_ROUND : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none")] } ;
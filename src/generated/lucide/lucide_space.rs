use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" ></ path > < / > } } pub const LucideSpace : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;
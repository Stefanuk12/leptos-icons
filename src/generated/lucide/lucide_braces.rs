use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1" ></ path > < path d = "M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1" ></ path > < / > } } pub const LucideBraces : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24")] } ;
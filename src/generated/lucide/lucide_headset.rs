use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 11h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-5Zm0 0a9 9 0 1 1 18 0m0 0v5a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3Z" ></ path > < path d = "M21 16v2a4 4 0 0 1-4 4h-5" ></ path > < / > } } pub const LUCIDE_HEADSET : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;
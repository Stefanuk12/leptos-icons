use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3.5" ></ path > < path d = "M4.017 11.512a6 6 0 1 0 8.466 8.475" ></ path > < path d = "M8 16v-6a6 6 0 0 1 6 6z" ></ path > < / > } } pub const LUCIDE_FILE_PIE_CHART : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
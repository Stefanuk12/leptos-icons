use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" rx = "2" width = "8" y = "8" x = "8" ></ rect > < path d = "M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2" ></ path > < path d = "M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2" ></ path > < / > } } pub const LUCIDE_BRING_TO_FRONT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
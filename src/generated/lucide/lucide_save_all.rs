use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 2v3a1 1 0 0 0 1 1h5" ></ path > < path d = "M18 18v-6a1 1 0 0 0-1-1h-6a1 1 0 0 0-1 1v6" ></ path > < path d = "M18 22H4a2 2 0 0 1-2-2V6" ></ path > < path d = "M8 18a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9.172a2 2 0 0 1 1.414.586l2.828 2.828A2 2 0 0 1 22 6.828V16a2 2 0 0 1-2.01 2z" ></ path > < / > } } pub const LUCIDE_SAVE_ALL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" ></ path > < path d = "M12 10v6" ></ path > < path d = "m9 13 3-3 3 3" ></ path > < / > } } pub const LUCIDE_FOLDER_UP : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;
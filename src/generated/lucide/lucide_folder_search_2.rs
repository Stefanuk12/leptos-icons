use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11.5" cy = "12.5" r = "2.5" ></ circle > < path d = "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" ></ path > < path d = "M13.3 14.3 15 16" ></ path > < / > } } pub const LUCIDE_FOLDER_SEARCH_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none")] } ;
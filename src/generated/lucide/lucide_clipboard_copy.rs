use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "1" rx = "1" height = "4" y = "2" x = "8" width = "8" ></ rect > < path d = "M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2" ></ path > < path d = "M16 4h2a2 2 0 0 1 2 2v4" ></ path > < path d = "M21 14H11" ></ path > < path d = "m15 10-4 4 4 4" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_COPY : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;
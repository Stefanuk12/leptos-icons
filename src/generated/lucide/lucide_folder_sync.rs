use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v.5" ></ path > < path d = "M12 10v4h4" ></ path > < path d = "m12 14 1.535-1.605a5 5 0 0 1 8 1.5" ></ path > < path d = "M22 22v-4h-4" ></ path > < path d = "m22 18-1.535 1.605a5 5 0 0 1-8-1.5" ></ path > < / > } } pub const LUCIDE_FOLDER_SYNC : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;
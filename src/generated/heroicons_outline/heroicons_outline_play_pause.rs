use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M21 7.5V18M15 7.5V18M3 16.811V8.69c0-.864.933-1.406 1.683-.977l7.108 4.061a1.125 1.125 0 0 1 0 1.954l-7.108 4.061A1.125 1.125 0 0 1 3 16.811Z" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLAY_PAUSE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;
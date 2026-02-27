// Token Shield - Interactive JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Waitlist Modal functionality
    const waitlistBtn = document.getElementById('waitlistBtn');
    const modal = document.getElementById('waitlistModal');
    const closeModal = document.getElementById('closeModal');
    const modalForm = document.getElementById('modalForm');
    
    // Open modal
    if (waitlistBtn && modal) {
        waitlistBtn.addEventListener('click', function(e) {
            e.preventDefault();
            modal.classList.add('active');
            document.body.style.overflow = 'hidden';
        });
    }
    
    // Close modal
    if (closeModal && modal) {
        closeModal.addEventListener('click', function() {
            modal.classList.remove('active');
            document.body.style.overflow = '';
        });
    }
    
    // Close modal when clicking overlay
    if (modal) {
        const overlay = modal.querySelector('.modal-overlay');
        if (overlay) {
            overlay.addEventListener('click', function() {
                modal.classList.remove('active');
                document.body.style.overflow = '';
            });
        }
    }
    
    // Close modal on Escape key
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && modal && modal.classList.contains('active')) {
            modal.classList.remove('active');
            document.body.style.overflow = '';
        }
    });
    
    // Consolidated form submission handler
    function handleWaitlistSubmit(form, shouldCloseModal = false) {
        const email = form.querySelector('input[name="email"], input[type="email"]').value;
        const twitter = form.querySelector('input[name="twitter"], input[type="text"]').value;
        const acceptsRisks = form.querySelector('input[name="acceptRisk"], input[type="checkbox"]').checked;
        
        if (!acceptsRisks) {
            alert('⚠️  You must accept the risks to join the waitlist.');
            return false;
        }
        
        // Show success message
        alert(`✓ SUCCESS\n\nYou're on the list, degen!\n\nWe'll notify ${email} when Token Shield launches.\n\n🛡️  Stay safe out there.`);
        
        // Close modal if needed
        if (shouldCloseModal && modal) {
            modal.classList.remove('active');
            document.body.style.overflow = '';
        }
        
        // Reset form
        form.reset();
        return true;
    }
    
    // Handle modal form submission
    if (modalForm) {
        modalForm.addEventListener('submit', function(e) {
            e.preventDefault();
            handleWaitlistSubmit(this, true);
        });
    }

    // Handle CTA form in contact section
    const ctaForm = document.querySelector('.cta-section .terminal-form');
    if (ctaForm) {
        ctaForm.addEventListener('submit', function(e) {
            e.preventDefault();
            handleWaitlistSubmit(this, false);
        });
    }

    // Smooth scroll for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            if (href !== '#' && href.length > 1) {
                e.preventDefault();
                const target = document.querySelector(href);
                if (target) {
                    const navHeight = document.querySelector('.terminal-nav') ? 
                        document.querySelector('.terminal-nav').offsetHeight : 0;
                    const targetPosition = target.offsetTop - navHeight - 20;
                    
                    window.scrollTo({
                        top: targetPosition,
                        behavior: 'smooth'
                    });
                }
            }
        });
    });

    // Console easter egg for degens
    console.log('%c🛡️ Token Shield', 'color: #00ff00; font-size: 24px; font-weight: bold;');
    console.log('%cGM degen! Thanks for checking out the code.', 'color: #00ff00; font-size: 14px;');
    console.log('%cWant to contribute? We are looking for smart contract devs, frontend wizards, and quantitative minds.', 'color: #808080; font-size: 12px;');
    console.log('%cJoin our Discord (coming soon) or hit us on Twitter!', 'color: #808080; font-size: 12px;');
});

// Documentation page sidebar navigation
if (document.querySelector('.docs-sidebar')) {
    const sidebarLinks = document.querySelectorAll('.sidebar-link');
    const sections = document.querySelectorAll('.doc-section[id]');
    
    // Update active link on scroll
    function updateActiveLink() {
        let currentSection = '';
        const scrollPosition = window.scrollY + 150;
        
        sections.forEach(section => {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
                currentSection = section.getAttribute('id');
            }
        });
        
        sidebarLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === `#${currentSection}`) {
                link.classList.add('active');
            }
        });
    }
    
    // Smooth scroll for sidebar links
    sidebarLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetSection = document.getElementById(targetId);
            if (targetSection) {
                const offset = 100;
                const targetPosition = targetSection.offsetTop - offset;
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });
    
    // Listen for scroll events
    let scrollTimeout;
    window.addEventListener('scroll', function() {
        if (scrollTimeout) {
            window.cancelAnimationFrame(scrollTimeout);
        }
        scrollTimeout = window.requestAnimationFrame(updateActiveLink);
    });
    
    // Initial call
    updateActiveLink();
}
